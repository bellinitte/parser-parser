<main>
    <a class="logo" href=".">Parser-parser</a>
    <nav>
        <ul class="nav__links">
            <li><a href="https://github.com/karolbelina/parser-parser"><i class="fa fa-github"></i></a></li>
        </ul>
    </nav>
</main>

<style>
    * {
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        background-color: rgb(30, 30, 30);
    }

    .logo {
        font-family: "JetBrains Mono", Consolas, monospace;
        font-size: 16px;
        color: #c9bcb0;
        text-decoration: none;
    }

    li a {
        font-size: 24px;
        color: #c9bcb0;
        text-decoration: none;
    }

    main {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 12px 16px;
    }

    .logo {
        cursor: pointer;
    }

    .nav__links {
        list-style: none;
    }

    .nav__links {
        display: inline-block;
        padding: 0 0 0 20px;
    }

    .nav__links li a, .logo {
        transition: all 0.3s ease 0s;
    }

    .nav__links li a:hover, .logo:hover {
        color: #ffffff;
    }
</style>
