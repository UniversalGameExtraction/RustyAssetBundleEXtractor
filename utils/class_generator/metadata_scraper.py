from __future__ import annotations
from dataclasses import dataclass, field
import json
from multiprocessing.pool import ThreadPool
import os


import requests
from bs4 import BeautifulSoup as Soup


from config import METADATA_PATH, SCRIPT_REFERENCE_CACHE_PATH


def main_scrap_metadata():
    # the toc.js file contains a list of all classes, their links and their children
    js_code = requests.get(
        "https://docs.unity3d.com/ScriptReference/docdata/toc.js"
    ).text
    # toc = {} -> UnityClass
    toc_raw = js_code[js_code.find("{", 1) : js_code.rfind("}", 1) + 1]
    toc = json.loads(toc_raw)

    classes = []
    stack = [([item["title"]], item["children"]) for item in toc["children"]]
    while stack:
        path, children = stack.pop()
        for child in children:
            child_path = path + [child["title"]]
            if child["children"]:
                stack.append((child_path, child["children"]))

            if child["link"] != "null":
                classes.append(UnityClass(child["title"], child["link"], child_path))

    ThreadPool(os.cpu_count()).map(UnityClass.get_metadata, classes)
    # for i, clz in enumerate(classes):
    #     print(f"{i+1}/{len(classes)}: {clz.title} ({clz.link})")
    #     clz.get_metadata()

    with open(METADATA_PATH, "wt", encoding="utf8") as f:
        json.dump([c.to_dict() for c in classes], f, ensure_ascii=False)
    print()


@dataclass
class UnityClass:
    title: str
    link: str
    path: list[str] = field(default_factory=list)
    description: str = ""
    properties: dict[str, str] = field(default_factory=dict)
    public_methods: dict[str, str] = field(default_factory=dict)
    static_methods: dict[str, str] = field(default_factory=dict)
    messages: dict[str, str] = field(default_factory=dict)
    constructors: dict[str, str] = field(default_factory=dict)
    delegates: dict[str, str] = field(default_factory=dict)
    operators: dict[str, str] = field(default_factory=dict)

    def __repr__(self) -> str:
        return f"{self.title} ({self.link})"

    @classmethod
    def from_dict(cls, data: dict) -> UnityClass:
        return cls(**data)

    def to_dict(self) -> dict:
        return self.__dict__

    def get_html(self, version: str = None) -> str:
        fp = os.path.join(SCRIPT_REFERENCE_CACHE_PATH, f"{self.link}.html")
        if os.path.exists(fp):
            with open(fp, "rt", encoding="utf8") as f:
                return f.read()

        if version is None:
            url = f"https://docs.unity3d.com/ScriptReference/{self.link}.html"
        else:
            url = f"https://docs.unity3d.com/{version}/Documentation/ScriptReference/{self.link}.html"

        res = requests.get(url).text

        os.makedirs(os.path.dirname(fp), exist_ok=True)
        with open(fp, "wt", encoding="utf8") as f:
            f.write(res)

        return res

    def get_metadata(self):
        # Expected format:
        # <div class="subsection">
        #     <h3>Static Properties</h3>
        #     <table class="list">
        #         <tr><td class="lbl"><a href="Texture2D-blackTexture.html">blackTexture</a></td><td class="desc">Gets a small Texture with all black pixels.</td></tr>
        #         ....
        #     </table>
        # </div>
        # the description is an exception, as it consists of two subsections, with the 2nd one being optional and only having a <p>
        soup = Soup(self.get_html(), "html.parser")
        subsections = soup.find_all("div", class_="subsection")

        description = ""
        for subsection in subsections:
            title = subsection.find("h3")
            if subsection.find("pre", class_="codeExampleCS"):
                continue

            if title is None:
                if description:
                    description += "\n" + subsection.find("p").text.strip()
                continue

            title = title.text.strip()
            if title == "Description":
                description = subsection.find("p").text.strip()
            else:
                data = getattr(self, title.lower().replace(" ", "_"), None)
                if data is None:
                    continue
                data.update(
                    {
                        row.contents[0].text.strip(): row.contents[1].text.strip()
                        for row in subsection.find_all("tr")
                    }
                )
        self.description = description


# def preserve_links(html: str, version: str) -> str:
#     soup = Soup(html, "html.parser")
#     for a in soup.find_all("a"):
#         if a["href"].startswith("http"):
#             continue
#         a["href"] = f"https://docs.unity3d.com/{version}/Documentation/ScriptReference/{a['href']}.html"
#     return str(soup

if __name__ == "__main__":
    main_scrap_metadata()
