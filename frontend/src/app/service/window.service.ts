import {ComponentRef, Injectable, Type, ViewContainerRef} from '@angular/core';
import {WindowContent} from "../data/window-content";
import {WindowFrame} from "../data/window-frame";
import {DraggableDirective} from "../directives/draggable.directive";
import {OptionMap} from "option-value";
import {None, Option, Some} from "option-value";
import {OptionArray} from "option-value";

@Injectable({
  providedIn: 'root'
})
export class WindowService {

  public vcr: Option<ViewContainerRef> = None()

  private currentWindowId: number = 0

  public openedApplications: OptionMap<number, OpenedApplication> = new OptionMap<number, OpenedApplication>()
  public focusStack: OptionArray<number> = new OptionArray<number>()

  constructor() { }

  public setDisplay(display: ViewContainerRef) {
    this.vcr = Some(display)
  }

  public openApplication
  <P, F extends WindowFrame, T extends WindowContent<P, F>>
  (componentType: Type<T>, params: P, frame: Type<F>): Option<number> {
    return this.vcr.map((vcr) => {
      let window = vcr.createComponent(frame)
      let vcrLoc = vcr.element.nativeElement as HTMLDivElement

      vcrLoc.appendChild(window.location.nativeElement)

      let windowContent = vcr.createComponent(componentType);
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

      return this.currentWindowId
    })
  }

  closeApplication(id: number) {
    this.openedApplications
      .maybeGet(id)
      .ifPresent((app) => {
        app.close()
        this.focusStack.splice(this.focusStack.findIndex(index => index == id),1)
        this.openedApplications.delete(id)
      })

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
      this.openedApplications
        .maybeGet(i)
        .ifPresent(app => app.setZIndex(this.focusStack.length - index))
    })
  }

  focusTop() {
    if (this.focusStack[0] != null) {
      this.focusApplication(this.focusStack[0])
    }
  }

  setPosition(id: number, x: number, y: number, center: boolean = false) {
    this.openedApplications
      .maybeGet(id)
      .ifPresent((app) => app.setPosition(x, y, center))
  }

  getDisplaySize(): Option<[number, number]> {
    return this.vcr
      .map(vcr => {
        let rect = (vcr.element.nativeElement as HTMLDivElement).getBoundingClientRect()
        return [rect.width, rect.height]
      })
  }

  makeDraggable(id: number) {
    this.openedApplications
      .maybeGet(id)
      .ifPresent((app) => {
        if (this.vcr.isSome()) {
          let dir = new DraggableDirective(app.window.location, this.vcr.get().element)
          dir.initDrag()
        }
      })
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
