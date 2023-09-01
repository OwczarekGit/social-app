export class PopupParams {
  type: PopupType
  text: string
  title: string


  constructor(type: PopupType, text: string, title: string) {
    this.type = type;
    this.text = text;
    this.title = title;
  }
}

export enum PopupType {
  Warning,
  Info ,
  Error,
}
