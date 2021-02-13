# POB Passive Skill Tree Extractor

Small command line app that extracts [Path Of Exile passive skill tree](https://www.pathofexile.com/passive-skill-tree) url  from the [Path Of Building Community Edition](https://github.com/PathOfBuildingCommunity/PathOfBuilding) exports.

## Usage

The app takes the POB base64 export to stdin and outputs the url.

```
cat sample/witch-build | cargo run

https://www.pathofexile.com/passive-skill-tree/AAAABAM....(rest of the URL omitted)
```

Open the url in browser and enjoy browsing the skill tree without access to POB.

## Roadmap

- [ ] make the app take pastebin as parameter instead of base64 encoded content 
- [ ] create a web version instead of cmd line app (compile to webassembly)
- [ ] extract latest version of the skill tree by default
- [ ] configure which version of skill tree to extract

## Notes

- tries to extract a skill tree of version 3_13