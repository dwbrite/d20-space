# DnD Greenroom

Ever need _some_ interaction and tracking in your dnd campaign, 
but not full-blown Roll20 + DnDBeyond? Hate having to duckduckgo or google spells a

## Requirements + Building

- Rust >= 1.59 (via rustup)
- Node (via your package manager of choice - just for npm)

Frontend: `cd ui && npm run watch`
Backend: `cargo run` (make sure your env vars are set)

## Usage

// TODO: mdBook usage docs

Select your character and enter your pin code.  
You're greeted with the game map and the menu.

### Environment Variables

With examples...

```
*BUCKET_SECRET=asdf91234yuhnkldm091mdasdlkaskld
*BUCKET_ACCESS=ASDFD8912U10SDF
*BUCKET_NAME=dnd-dnd-dnd
*BUCKET_REGION=us-east-1
*BUCKET_ENDPOINT=https://us-east-1.linodeobjects.com
```

`* = required`