// vim: ts=4 et sw=4

// Prefill searchbox with the previous search in the search results page.
const prefillSearchBox = () => {
    el = document.querySelector("input[name=search]")
    addr = document.URL
    let re = /.*\/search\?search=(.*)/
    let matches = re.exec(addr)
    if (matches) {
        el.value = decodeURIComponent(matches[1].replace("+", " "))
    }
}
prefillSearchBox();


// Make verses paintable.

const versePainter = () => {
    const painter = (event) => {
        if (event.buttons & 2 && event.target.classList.contains("hilight")) {
            event.target.classList.remove("hilight")
        } else if (event.buttons & 2) {
            event.target.classList.add("hilight")
        }
    }
    const clicked = (event) => {
        if (event.target.classList.contains("hilight")) {
            event.target.classList.remove("hilight")
        } else {
            event.target.classList.add("hilight")
        }
    }
    els = document.querySelectorAll(".verse")
    for (let el of els) {
        el.addEventListener("mouseenter", painter)
        el.addEventListener("click", clicked)
    }
}
versePainter()

// Make custom context menu for the chapter view.
const versesElement = document.querySelector("#chapter-container")
const copyLinkMenuItem = document.querySelector("#copy-link")
const makeAndCopyLink = (event) => {
    const infoGrabber = /(https?:\/\/.+\/books\/)(.+\/)(\d+).*/
    const matches = infoGrabber.exec(document.URL)
    let verses = document.querySelectorAll(".hilight")
    let result = "?"
    for (let verse of verses) {
        result += `hls=${verse.querySelector(".verse-nr").innerHTML}&`
    }
    let url = `${matches[1]}${matches[2]}${matches[3]}${result}`
    navigator.clipboard.writeText(url)
}
const preventNewMenu = (event) => {
    event.preventDefault()
    return false
}
const showMenu = (event) => {
    event.preventDefault()
    let ctxMenu = document.querySelector("#context-menu")
    let ctxCopyLink = document.querySelector("#copy-link")
    ctxMenu.style.display = "block"
    ctxMenu.style.left = `${event.pageX}px`
    ctxMenu.style.top = `${event.pageY}px`
    versesElement.removeEventListener("contextmenu", showMenu)
    versesElement.addEventListener("contextmenu", preventNewMenu)
    document.addEventListener("click", hideMenu)
    ctxCopyLink.addEventListener("click", makeAndCopyLink)
}
const hideMenu = (event) => {
    let ctxMenu = document.querySelector("#context-menu")
    ctxMenu.style.display = "none"
    document.removeEventListener("click", hideMenu)
    versesElement.addEventListener("contextmenu", showMenu)
}
versesElement.addEventListener("contextmenu", showMenu)

