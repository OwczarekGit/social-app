import {ComponentRef, Injectable, Type, ViewContainerRef} from '@angular/core';
import {RegistrationFormComponent} from "../forms/registration-form/registration-form.component";
import {WindowComponent} from "../ui-elements/window/window.component";

@Injectable({
  providedIn: 'root'
})
export class WindowService {

  public surface!: ViewContainerRef

  public currentId: number = 0
  public openedWindows: Map<number, WindowComponent> = new Map()
  private refs: ComponentRef<any>[] = []

  constructor() { }

  public setSurface(vcr: ViewContainerRef) {
    this.surface = vcr
    this.surface.element.nativeElement.style.position = 'relative'
  }

  register(win: WindowComponent) {
    this.openedWindows.set(this.currentId, win)
    win.id = this.currentId

    this.currentId++
  }

  public openApplication(component: Type<any>): ComponentRef<any> {
    let element = this.surface.createComponent(component)
    this.refs.push(element)
    this.surface.element.nativeElement.appendChild(element.location.nativeElement)

    return element
  }

  public setPosition(id: number, x: number, y: number) {
    let win = this.openedWindows.get(id)
    if (win == undefined) return

    let el = win.host.element.nativeElement as HTMLDivElement
    el.style.left = `${x}px`
    el.style.top  = `${y}px`
  }

  close(id: number) {
    let win = this.openedWindows.get(id)
    if (win == undefined) return

    let index = this.refs.findIndex((r) => (r.instance['window'] as WindowComponent).id == id)
    if (index == -1) return

    this.refs[index].destroy()
  }

  public getSurfaceSize(): [number, number] {
    let size = this.surface.element.nativeElement.getBoundingClientRect()
    return [size.width, size.height]
  }
}
