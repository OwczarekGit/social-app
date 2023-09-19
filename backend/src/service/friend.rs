use std::sync::Arc;
use axum_macros::FromRef;
use neo4rs::{query, Graph, Node, Row};
use serde::{Deserialize, Serialize};
use crate::{Result, Error};

#[derive(Clone, FromRef)]
pub struct FriendService {
    neo4j: Arc<Graph>,
}

impl FriendService {

    pub async fn remove_friend(&self, user_id: i64, other_id: i64) -> Result<()> {
        self.neo4j.run(
            query("match (n:Profile{id: $id})-[r:FRIEND]-(o:Profile{id: $other_id}) delete r")
                .param("id", user_id)
                .param("other_id", other_id)
        ).await?;

        Ok(())
    }

    pub async fn get_friend_list(&self, user_id: i64) -> Result<Vec<Profile>> {
        let q = query("match (m:Profile{id: $id})-[:FRIEND]-(p:Profile) return p")
            .param("id", user_id);

        let mut results = self.neo4j.execute(q)
            .await?;


        let mut res = vec![];
        while let Some(row) = results.next().await? {
            if let Ok(profile) = Profile::try_from(row) {
                res.push(profile)
            }
        }

        Ok(res)
    }

    pub async fn search_for_non_friends(&self, user_id: i64, phrase: &str) -> Result<Vec<SearchNonFriendsResult>> {
        let search_query = query(r#"
            match (p:Profile{id: $id}), (p2:Profile)
            where (not (p)-[:FRIEND]-(p2) and not (p)-[:REQUESTED_FRIENDSHIP]-(p2))
            and (p2.id <> $id and toLower(p2.username) contains toLower($phrase))
            return p2"#)
            .param("id", user_id)
            .param("phrase", phrase);

        let mut results = self.neo4j.execute(search_query)
            .await?;

        let mut res = vec![];
        while let Ok(Some(x)) = results.next().await {
            let n: Node = x.get("p2").ok_or(Error::Neo4jNodeNotFound)?;

            let id: i64 = n.get("id").ok_or(Error::Neo4jInvalidNode(n.id()))?;
            let username: String = n.get("username").ok_or(Error::Neo4jInvalidNode(n.id()))?;
            res.push(SearchNonFriendsResult {
                user_id: id,
                username
            });
        }

        Ok(res)
    }

    pub async fn get_pending_friend_requests(&self, user_id: i64) -> Result<Vec<FriendRequest>> {
        let query = query("match (p:Profile)-[:REQUESTED_FRIENDSHIP]->(:Profile{id: $id}) return p")
            .param("id", user_id);

        let mut data = self.neo4j.execute(query).await?;

        let mut requests = vec![];
        while let Ok(Some(row)) = data.next().await {
            let n: Node = row.get("p").ok_or(Error::Neo4jNodeNotFound)?;

            let id: i64 = n.get("id").ok_or(Error::Neo4jInvalidNode(n.id()))?;
            let username: String = n.get("username").ok_or(Error::Neo4jInvalidNode(n.id()))?;

            requests.push(FriendRequest{
                user_id: id,
                username
            });
        }

        Ok(requests)
    }

    pub async fn accept_friend_request(&self, user_id: i64, requester_id: i64) -> Result<()> {
        let query = query("match (p1:Profile {id: $requester_id})-[r:REQUESTED_FRIENDSHIP]-(p2:Profile {id: $user_id}) delete r merge (p1)-[:FRIEND]->(p2)")
            .param("requester_id", requester_id)
            .param("user_id", user_id);

        self.neo4j.run(query).await?;
        Ok(())
    }

    pub async fn send_friend_request(&self, user_id: i64, target_id: i64) -> Result<()> {
        let is_already_friend_query = query(r#"
                return exists
                ((:Profile{id: $user_id})-[:FRIEND]-(:Profile{id: $target_id})) or
                ((:Profile{id: $user_id})-[:REQUESTED_FRIENDSHIP]-(:Profile{id: $target_id}))
                as result
            "#)
            .param("user_id", user_id)
            .param("target_id", target_id);

        let is_already_friend = self.neo4j.execute(is_already_friend_query)
            .await?
            .next()
            .await?
            .ok_or(Error::Neo4jNodeNotFound)?
            .get::<bool>("result")
            .ok_or(Error::Neo4jQueryError)?;

        if is_already_friend {
            return Err(Error::RelationErrorIsAlreadyFriend(user_id, target_id));
        }

        let create_request_query = query("match (m:Profile{id: $id}), (p:Profile{id: $target}) merge (m)-[:REQUESTED_FRIENDSHIP]->(p)")
            .param("id", user_id)
            .param("target", target_id);

        self.neo4j.run(create_request_query)
            .await?;

        Ok(())
    }
}

impl FriendService {
    pub fn new(neo4j: Arc<Graph>) -> Self {
        Self {neo4j}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FriendRequest {
    user_id: i64,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchNonFriendsResult {
    user_id: i64,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    user_id: i64,
    username: String,
    picture_url: String,
}

impl TryFrom<Row> for Profile {
    type Error = Error;

    fn try_from(row: Row) -> Result<Self> {
        let n: Node = row.get("p").ok_or(Error::Neo4jNodeNotFound)?;

        Ok(Self {
            user_id: n.get("id").ok_or(Error::Neo4jInvalidNode(n.id()))?,
            username: n.get("username").ok_or(Error::Neo4jInvalidNode(n.id()))?,
            picture_url: n.get("picture_url").unwrap_or("".to_string())
        })
    }
}