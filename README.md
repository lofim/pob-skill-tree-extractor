# POB Passive Skill Tree Extractor

Small command line app that extracts [Path Of Exile passive skill tree](https://www.pathofexile.com/passive-skill-tree) url  from the [Path Of Building Community Edition](https://github.com/PathOfBuildingCommunity/PathOfBuilding) exports.

## Usage

```bash
# Provide pastebin URL using the -p parameter
~ cargo run -- -p https://pastebin.com/YvdyACrG
https://www.pathofexile.com/passive-skill-tree/AAAABAM....(rest of the URL omitted)

# Pipe POB export directly to extractor
~ cat sample/witch-build | cargo run
https://www.pathofexile.com/passive-skill-tree/AAAABAM....(rest of the URL omitted)
```

Open the url in browser and enjoy browsing the skill tree without access to POB.

## Roadmap

- [x] make the app take pastebin as parameter instead of base64 encoded content
- [x] extract latest version of the skill tree by default
- [ ] configure which version of skill tree to extract
- [ ] create a web version instead of cmd line app (compile to webassembly)


## Notes

- tested with skill tree of version 3_13
