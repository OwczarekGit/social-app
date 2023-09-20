export class Post {
  id: number
  author_id: number
  author_username: string
  author_picture_url: string
  content: string
  date: Date


  constructor(id: number, author_id: number, author_username: string, author_picture_url: string, content: string, date: Date, domain: string) {
    this.id = id;
    this.author_id = author_id;
    this.author_username = author_username;
    this.author_picture_url = domain + author_picture_url;
    this.content = content;
    this.date = date;
  }
}
