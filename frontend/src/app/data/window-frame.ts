import {signal} from "@angular/core";

export abstract class WindowFrame {
  title = signal<string>('')
  iconUrl = signal<string>('')

  abstract putContent(content: HTMLDivElement): void

  onFocus!: () => void
}
