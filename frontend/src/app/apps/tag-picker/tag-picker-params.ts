import {Tag} from "../../data/tag";

export class TagPickerParams {

  currentTags: Tag[]
  resultTags: (tags: Tag[]) => void


  constructor(currentTags: Tag[], resultTags: (tags: Tag[]) => void) {
    this.currentTags = currentTags
    this.resultTags = resultTags
  }
}
