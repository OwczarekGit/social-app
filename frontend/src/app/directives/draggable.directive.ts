import {AfterViewInit, Directive, ElementRef} from '@angular/core';
import {fromEvent, Subscription, takeUntil} from "rxjs";


@Directive({
  selector: '[wm_draggable]'
})
export class DraggableDirective implements AfterViewInit {

  private element: ElementRef<any>


  constructor(element: ElementRef<any>) {
    this.element = element;
  }

  ngAfterViewInit(): void {
    this.initDrag()
  }

  public initDrag() {
    let el = this.element.nativeElement as HTMLElement
    let dragStart = fromEvent<MouseEvent>(el, "mousedown")
    let dragEnd = fromEvent<MouseEvent>(el, "mouseup")
    let drag = fromEvent<MouseEvent>(el, "mousemove").pipe(takeUntil(dragEnd))

    let initX: number
    let initY: number
    let currentX: number = this.parseNum(el.style.left)
    let currentY: number = this.parseNum(el.style.top)

    let dragSub: Subscription

    dragStart.subscribe(ev => {
      initX = ev.clientX - this.parseNum(el.style.left)
      initY = ev.clientY - this.parseNum(el.style.top)

      if (!(ev.target as HTMLElement).classList.contains("dw_draggable")) return

      el.style.cursor = 'move'

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
