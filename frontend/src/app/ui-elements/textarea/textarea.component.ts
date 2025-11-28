import {Component, EventEmitter, forwardRef, Input, Output} from '@angular/core';
import {ControlValueAccessor, NG_VALUE_ACCESSOR} from "@angular/forms";

@Component({
    selector: 'app-textarea',
    templateUrl: './textarea.component.html',
    styleUrls: ['./textarea.component.css'],
    providers: [
        {
            provide: NG_VALUE_ACCESSOR,
            useExisting: forwardRef(() => TextareaComponent),
            multi: true
        }
    ],
    standalone: false
})
export class TextareaComponent implements ControlValueAccessor {
  @Input() text!: string
  @Output() textChange = new EventEmitter<string>()

  onChange: any = () => {
    this.textChange.emit(this.text)
  }
  onTouch: any = () => {}
  registerOnChange(fn: any): void {
    this.onChange = fn;
  }

  registerOnTouched(fn: any): void {
    this.onTouch = fn;
  }

  writeValue(input: string) {
    this.text = input
  }

}
