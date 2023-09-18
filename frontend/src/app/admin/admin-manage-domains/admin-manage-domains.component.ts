import {AfterViewInit, Component, computed, effect, inject, OnInit, signal} from '@angular/core';
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {AdminDomain} from "./admin-domain";
import {ListDisplay} from "../../data/list-display";
import {DomainService} from "../../service/domain.service";
import {PopupService} from "../../service/popup.service";

@Component({
  selector: 'app-admin-manage-domains',
  templateUrl: './admin-manage-domains.component.html',
  styleUrls: ['./admin-manage-domains.component.css']
})
export class AdminManageDomainsComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit, OnInit {

  private domainService = inject(DomainService)
  private popupService = inject(PopupService)

  domains: AdminDomain[] = [
    new AdminDomain("Image domain"),
    new AdminDomain("System domain"),
  ]

  inputValue: string = ''

  selectedDomain = signal<AdminDomain | null>(null)

  getDomainVariable = effect(() => {
    let val = this.selectedDomain()
    if (val == null) return

    this.inputValue = ''

    switch (val.value) {
      case "Image domain": {
        this.domainService.getImageDomain().subscribe({
          next: value => {
            if (value != null) {
              this.inputValue = value.value
            }
          }
        })
      } break;
      case "System domain": {
        this.domainService.getSystemDomain().subscribe({
          next: value => {
            if (value != null) {
              this.inputValue = value.value
            }
          }
        })
      } break;
    }
  })

  ngAfterViewInit(): void {
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
    this.windowFrame.onClose = () => this.closeWindow()
    setTimeout(() => {
      this.setTitle("Mange domains")
      this.setIcon("/assets/manage-domain-s.png")
    })

  }

  selectDomain($event: ListDisplay) {
    this.selectedDomain.set($event as AdminDomain)
  }

  ngOnInit(): void {
    this.selectedDomain.set(this.domains[0])
  }

  update() {
    let domain = this.selectedDomain()
    if (domain == null) return

    switch (domain.value) {
      case "Image domain": {
        this.domainService.setImageDomain(this.inputValue).subscribe({
          next: _ => {
            this.popupService.info(
              "Image domain updated",
              "The image domain has been updated."
            )
          },
          error: _ => {
            this.popupService.error(
              "Image domain update error",
              "There was an error and the image domain has not been updated. Are you an admin?"
            )
          }
        })
      } break;
      case "System domain": {
        this.domainService.setSystemDomain(this.inputValue).subscribe({
          next: _ => {
            this.popupService.info(
              "System domain updated",
              "The system domain has been updated."
            )
          },
          error: _ => {
            this.popupService.error(
              "System domain update error",
              "There was an error and the system domain has not been updated. Are you an admin?"
            )
          }
        })
      } break;
    }
  }
}
