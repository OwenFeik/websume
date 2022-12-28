// Set up tabbed view
document.querySelectorAll(".tab").forEach(tab => {
    let tab_content = document.querySelector(
        ".tab_content[data-tab='" + tab.getAttribute("data-tab") + "']"
    );
    tab.onclick = () => {
        document.querySelector(".tab_content.active").classList.remove(
            "active"
        );
        document.querySelector(".tab.active").classList.remove("active");
        tab_content.classList.add("active");
        tab.classList.add("active");
    };
});
