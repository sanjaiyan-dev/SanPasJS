const sanjaiyanGuidesSideBar = [
  {
    text: "Introduction",
    items: [
      { text: "Overview", link: "/guides/what-is-sanpasjs" },
      {
        text: "Installation",
        items: [
          {
            text: "Basics",
            link: "/guides/installation/index",
          },
          {
            text: "Windows",
            link: "/guides/installation/windows",
          },
          {
            text: "Linux",
            link: "/guides/installation/linux",
          },
        ],
      },
    ],
  },
  {
    text: "CLI Commands",
    items: [
      {
        text: "new",
        link: "/guides/commands/new",
      },
      {
        text: "compile",
        link: "/guides/commands/compile",
      },
    ],
  },
  {
    text: "Special Function",
    items: [
      {
        text: "HTML Output",
        link: "/guides/special-funcs/html",
      },
      {
        text: "Text Output",
        link: "/guides/special-funcs/text",
      },
      {
        text: "Clear Output",
        link: "/guides/special-funcs/clear",
      },
    ],
  },
  {
    text: "Caveats",
    link: "/guides/caveats/index"
  },
];

export default {
  title: "SanPasJs",
  description:
    "Compile your Pascal program to Javascript program which can be executed in the web browsers 💻🕸💻",
  head: [
    [
      "meta",
      {
        name: "og:image",
        content: "https://sanpasjs.web.app/Sanjaiyan_Pascal_Js_OgImg_San.jpg",
      },
    ],
  ],
  themeConfig: {
    logo: "/Sanjaiyan_Pascal_To_Js_Logo.png",
    nav: [
      { text: "Home", link: "/" },
      { text: "Guides", link: "/guides/what-is-sanpasjs" },
      { text: "Team", link: "/team" },
      { text: "Download", link: "/download" },
    ],
    sidebar: {
      "/guides/": sanjaiyanGuidesSideBar,
    },
    socialLinks: [
      { icon: "instagram", link: "https://instagram.com/sanjaiyan_dev" },
    ],
  },
};
