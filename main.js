document.getElementById('footer-js').innerHTML = `
    <a href="../">Home</a>  —
    <a href="https://github.com/omar-elrefaei">Github</a> —
    <a href="https://linkedin.com/in/omar-elrefaei/">Linkedin</a>
    <span class="aestheticbreak"></span>
    <span class="aestheticbreak"></span>
    <span class="aestheticbreak"></span>
    <a href="mailto:el.omar.dev@gmail.com">Contact me at: el.omar.dev@gmail.com</a>
    <span class="aestheticbreak"></span>
    Site styled with care using <a href="https://missing.style/">missing.css</a>
    <p>&copy; ${new Date().getFullYear()} Omar Elrefaei — <em>Taming Computers • Crafting Code</em></p>
`;


// Copy blog title from title-tag to h1. One place rename 
var h1_title = document.getElementById("h1-title-js")
if (h1_title) {
    h1_title.innerHTML = document.title;
}


// Maintain URL parameters for analytics through internal links
var isExternal = function (url) {
    return !(location.href.replace("://", "").split("/")[0] === url.replace("://", "").split("/")[0]);
}
var queryString = new URL(window.location).search;
document.querySelectorAll("[href]").forEach(link => {
    if (!isExternal(link.href)) {
        var current = link.href;
        link.href = current + queryString;
    }
});