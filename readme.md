# About

phlex-emmet-ls is a language server that provides [Emmet](https://emmet.io/) completions for [Phlex](https://www.phlex.fun/) templates.

![demo](https://github.com/user-attachments/assets/261b9e4c-b9b8-48df-a2c2-ad52c7d60779)


## Instalation

Install the binary using cargo:

`cargo install phlex_emmet_ls`

### Neovim

The Neovim plugin can be found at [https://github.com/lrangell/phlex-emmet.nvim](https://github.com/lrangell/phlex-emmet.nvim)

### Using [lazy.nvim](https://github.com/folke/lazy.nvim)

```lua
{
  "lrangell/phlex-emmet.nvim",
}
```

## Implemented features

- [x] Child: >
- [x] Sibling: + - [x] Multiplication: \*
- [x] ID and CLASS attributes
- [x] Implicit tag names
- [x] Text: {}
- [ ] Climb-up: ^
- [ ] Item numbering: $
- [ ] Grouping: ()
