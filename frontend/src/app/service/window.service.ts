import {ComponentRef, Injectable, Type, ViewContainerRef} from '@angular/core';
import {WindowContent} from "../data/window-content";
import {WindowFrame} from "../data/window-frame";
import {DraggableDirective} from "../directives/draggable.directive";

@Injectable({
  providedIn: 'root'
})
export class WindowService {

  private vcr?: ViewContainerRef

  private currentWindowId: number = 0

  private openedApplications: Map<number, OpenedApplication> = new Map<number, OpenedApplication>()
  private focusStack: number[] = []

  constructor() { }

  public setDisplay(display: ViewContainerRef) {
    this.vcr = display
  }

  public openApplication
    <P, F extends WindowFrame, T extends WindowContent<P, F>>
    (componentType: Type<T>, params: P, frame: Type<F>) {

    if (this.vcr == null) {
      console.error("VCR has not been set.")
      return
    }

    let window = this.vcr.createComponent(frame)
    let vcrLoc = this.vcr.element.nativeElement as HTMLDivElement

    vcrLoc.appendChild(window.location.nativeElement)

    let windowContent = this.vcr?.createComponent(componentType);
    let el = windowContent?.location.nativeElement as HTMLDivElement

    if (windowContent?.instance != null) {
      windowContent.instance.id = ++this.currentWindowId;
      windowContent.instance.windowFrame = window.instance
    } else {
      console.error("Instance was null.")
    }

    windowContent?.instance.setParams(params)
    window.instance.putContent(el)

    this.focusStack.unshift(this.currentWindowId)
    this.openedApplications.set(this.currentWindowId, new OpenedApplication(window, windowContent))
    this.focusApplication(this.currentWindowId)
    this.makeDraggable(this.currentWindowId)
  }

  closeApplication(id: number) {
    let app = this.openedApplications.get(id)
    if (app != null) {
      app.close()
      this.focusStack.splice(this.focusStack.findIndex(index => index == id),1)
      this.openedApplications.delete(id)
    }

    this.focusTop()
    this.fixZIndex()
  }

  focusApplication(id: number) {
    this.openedApplications.forEach((app, i) => {
      if (i == id) {
        app.setFocusedState(true)
        let item = this.focusStack.splice(this.focusStack.findIndex(index => index == i),1)[0]
        this.focusStack.unshift(item)
      } else {
        app.setFocusedState(false)
      }
    })

    this.fixZIndex()
  }

  fixZIndex() {
    this.focusStack.forEach((i,index) => {
      let app = this.openedApplications.get(i)
      if (app != null)
        app.setZIndex(this.focusStack.length - index)
    })
  }

  focusTop() {
    if (this.focusStack[0] != null) {
      this.focusApplication(this.focusStack[0])
    }
  }

  setPosition(id: number, x: number, y: number, center: boolean = false) {
    let app = this.openedApplications.get(id)
    if (app != null) {
      app.setPosition(x, y, center)
    }
  }

  getDisplaySize(): [number, number] {
    if (this.vcr == null) {
      console.error("VCR not set before accessing.")
    }
    let rect = (this.vcr?.element.nativeElement as HTMLDivElement).getBoundingClientRect()
    return [rect.width, rect.height]
  }

  makeDraggable(id: number) {
    let app = this.openedApplications.get(id)
    if (app != null) {
      let dir = new DraggableDirective(app.window.location)
      dir.initDrag()
    }
  }
}



class OpenedApplication {
  window: ComponentRef<WindowFrame>
  content: ComponentRef<WindowContent<any, any>>

  constructor(window: ComponentRef<WindowFrame>, content: ComponentRef<WindowContent<any, any>>) {
    this.window = window;
    this.content = content;
  }

  close() {
    this.window.destroy()
  }

  setZIndex(i: number) {
    (this.window.location.nativeElement as HTMLDivElement).style.zIndex = `${i}`;
  }

  setPosition(x: number, y: number, center: boolean) {
    let el = (this.window.location.nativeElement as HTMLDivElement);
    if (center) {
      let rect = el.getBoundingClientRect()
      el.style.left = `${x-rect.width/2}px`
      el.style.top = `${y-rect.height/2}px`

    } else {
      el.style.left = `${x}px`
      el.style.top = `${y}px`
    }
  }

  setFocusedState(state: boolean) {
    if (state) {
      (this.window.location.nativeElement as HTMLDivElement).children[0].classList.add('wm_focused');
      (this.window.location.nativeElement as HTMLDivElement).children[0].classList.remove('wm_not-focused');
    } else {
      (this.window.location.nativeElement as HTMLDivElement).children[0].classList.add('wm_not-focused');
      (this.window.location.nativeElement as HTMLDivElement).children[0].classList.remove('wm_focused')
    }
  }
}
