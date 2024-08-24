// vim: ts=4 et sw=4

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

