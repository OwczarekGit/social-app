import {AfterViewInit, Directive, ElementRef, inject} from '@angular/core';
import {fromEvent, Subscription, takeUntil} from "rxjs";
import {WindowService} from "../service/window.service";


@Directive({
  selector: '[wm_draggable]'
})
export class DraggableDirective implements AfterViewInit {

  private element: ElementRef<any>
  private display: ElementRef<HTMLDivElement>

  constructor(element: ElementRef<any>, display: ElementRef<HTMLDivElement>) {
    this.element = element;
    this.display = display
  }

  ngAfterViewInit(): void {
    this.initDrag()
  }

  public initDrag() {
    let el = this.element.nativeElement as HTMLElement
    let dragStart = fromEvent<MouseEvent>(el, "mousedown")
    let dragEnd = fromEvent<MouseEvent>(el, "mouseup")
    let drag = fromEvent<MouseEvent>(this.display.nativeElement, "mousemove").pipe(takeUntil(dragEnd))

    let initX: number
    let initY: number
    let currentX: number = this.parseNum(el.style.left)
    let currentY: number = this.parseNum(el.style.top)

    let dragSub: Subscription

    dragStart.subscribe(ev => {
      initX = ev.clientX - this.parseNum(el.style.left)
      initY = ev.clientY - this.parseNum(el.style.top)

      if (!(ev.target as HTMLElement).classList.contains("wm_draggable")) return

      el.style.cursor = 'move'
      el.classList.add('wm_dragged')

      dragSub = drag.subscribe(ev => {
        ev.preventDefault()
        currentX = ev.clientX - initX
        currentY = ev.clientY - initY

        el.style.left = `${currentX}px`
        el.style.top = `${currentY}px`
      })

      let dragEndSub = dragEnd.subscribe(ev => {
        initX = currentX
        initY = currentY
        el.style.cursor = ""
        el.classList.remove('wm_dragged')
        if (dragSub)
          dragSub.unsubscribe()
      })
    })
  }

  private parseNum(num: string): number {
    let parsed = parseInt(num.replace("px", ""))
    if (isNaN(parsed)) {
      return 0
    }
    return parsed
  }

}
