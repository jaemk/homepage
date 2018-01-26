document.addEventListener('DOMContentLoaded', function() {
    var nav         = document.getElementById("-nav");
    var nav_toggle  = document.getElementById("-nav-toggle");
    var content     = document.getElementById("-content");

    var link_top    = document.getElementById("-top-link");
    var top         = document.getElementById("-top");
    var link_about  = document.getElementById("-about-link");
    var about       = document.getElementById("-about");
    var link_exp    = document.getElementById("-exp-link");
    var experience  = document.getElementById("-experience");
    var link_tech   = document.getElementById("-tech-link");
    var technology  = document.getElementById("-technology");
    var link_proj   = document.getElementById("-proj-link");
    var projects    = document.getElementById("-projects");
    var link_ed     = document.getElementById("-ed-link");
    var education   = document.getElementById("-education");
    var side_links = [
        [link_top, top],
        [link_about, about],
        [link_exp, experience],
        [link_tech, technology],
        [link_proj, projects],
        [link_ed, education],
    ];

    function width() {
        return window.innerWidth
            || document.documentElement.clientWidth
            || document.body.clientWidth;
    }

    var navThresh = 1020;
    side_links.forEach(function(links){
        links[1].scrollTop = 120;
        links[0].addEventListener("click", function() {
            links[1].scrollIntoView();
            if (width() < navThresh) {
                navClose();
            }
        });
    });

    function scrollToHash() {
        var hash = window.location.hash;
        if (hash) {
            switch (hash) {
                case "#top":
                    top.scrollIntoView();
                    break;
                case "#about":
                    about.scrollIntoView();
                    break;
                case "#experience":
                    experience.scrollIntoView();
                    break;
                case "#technology":
                    technology.scrollIntoView();
                    break;
                case "#education":
                     ducation.scrollIntoView();
                    break;
                case "#contact":
                    contact.scrollIntoView();
                    break;
                default:
                    break;
            }
        }
    }
    scrollToHash();

    var navClosed   = false;
    function navOpen() {
        nav.classList.remove("closed");
        content.classList.remove("full");
        navClosed = false;
        if (width() < navThresh) {
            content.classList.add("absolute-slide");
        }
    }
    function navClose() {
        nav.classList.add("closed");
        content.classList.add("full");
        navClosed = true;
        content.classList.remove("absolute-slide");
    }
    nav_toggle.addEventListener('click', function() {
        if (nav.classList.contains("closed")) {
            navOpen();
        } else {
            navClose();
        }
    })

    function handleResize() {
        var _width = width();

        if (_width < navThresh && !navClosed) {
            navClose();
        } else if (_width > navThresh && navClosed) {
            navOpen();
        }
    }
    // Handle initial page sizing
    handleResize();
    window.addEventListener("resize", handleResize);

});

