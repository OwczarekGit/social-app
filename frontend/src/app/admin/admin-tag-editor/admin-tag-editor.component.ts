import {AfterViewInit, Component, inject, signal, ViewChild} from '@angular/core';
import {DetailedTag} from "../../data/detailed-tag";
import {TagService} from "../../service/tag.service";
import {ListDisplay} from "../../data/list-display";
import {PopupService} from "../../service/popup.service";
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Component({
  selector: 'admin-tag-editor',
  templateUrl: './admin-tag-editor.component.html',
  styleUrls: ['./admin-tag-editor.component.css']
})
export class AdminTagEditorComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit {

  tagService = inject(TagService)
  popupService = inject(PopupService)

  searchString: string = ''
  newTagName: string = ''

  nowEditingTag?: DetailedTag

  filteredTags = signal<DetailedTag[]>([])
  allTags = signal<DetailedTag[]>([])

  constructor() {
    super()
    this.tagService.getAllDetailedTags().subscribe({
      next: value => {
        this.allTags.set(value.map(t => new DetailedTag(t.id, t.name, t.usage)))
        this.filteredTags.set(this.allTags())
      }
    })
  }

  ngAfterViewInit(): void {
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
    this.windowFrame.onClose = () => this.closeWindow()
    setTimeout(() => {
      this.setTitle("Mange tags")
      this.setIcon("/assets/tag-s.png")
    })
  }

  close() {
    this.closeWindow()
  }

  editTag($event: ListDisplay) {
    this.nowEditingTag = $event as DetailedTag
  }

  public filterTags() {
    if (this.searchString == '') {
      this.filteredTags.set(this.allTags())
    } else {
      this.filteredTags.set(this.allTags().filter(t => t.name.toLowerCase().trim().includes(this.searchString.toLowerCase().trim())))
    }
  }

  createNewTag() {
    this.tagService.createNewTag(this.newTagName).subscribe({
      next: tag => {
        this.allTags.update(t => [...t, new DetailedTag(tag.id, tag.name, tag.usage)])
        this.filteredTags.set(this.allTags())
        this.popupService.info("Tag created", "New tag has been created successfully.")
        this.newTagName = ''
      },
      error: _ => {
        this.popupService.error(
          "Error creating tag",
          "There was an error creating tag. Does the tag with that name exist already?")
      }
    })
  }

  deleteTag($event: DetailedTag) {
    this.searchString = ''
    this.tagService.deleteTag($event.id).subscribe({
      next: _ => {
        this.allTags.mutate(t => {
          t.splice(t.findIndex(tag => tag.id == this.nowEditingTag?.id),1)
        })

        this.nowEditingTag = undefined
        this.popupService.info("Tag removed", "The tag has been removed.")
      },
      error: _ => {
        this.popupService.error(
          "Tag removal failed",
          "There was an error removing tag.")
      }, complete: () => {
        this.filteredTags.set(this.allTags())
      }
    })
  }

  updateTag($event: DetailedTag) {
    this.searchString = ''
    this.tagService.updateTag($event.id, $event.name).subscribe({
      next: _ => {
        this.nowEditingTag = undefined
        this.popupService.info("Tag updated", "The tag has been updated.")
      },
      error: _ => {
        this.popupService.error(
          "Error",
          "Tag update failed. Either you don't have permission to perform this action, " +
          "or there is some problem server-side."
        )
      }, complete: () => {
        this.filteredTags.set(this.allTags())
      }
    })
  }
}
