# vimsheet
I'm learning Rust and Neovim right now, and I haven't been able to find a good vim cheat sheet generator... So I decided to make one of my own! VimSheet exports to a CSV file so you can do whatever you desire with the cheetsheet. Not right now though, it's still in development

# Using VIMSheet
All Neovim `init.vim` files should be compatible with VIMSheet.

To add a description to a map all you have to do is simply add a comment at the end of the line using a `"` and then whatever your description is!

# Example
You can see on the first line there is a comment with `!exclude`. This tells VIMSheet not to include that line when processing. If you are getting a false positive on a line for some reason, then you can add this exclude tag and it will stop including it.
Note that VIMSheet checks for `!exclude` anywhere in a line. If a line isn't showing up, it's probably because of that.
```vim
nnoremap <Space> <NOP> " !exclude

" Commands
nnoremap <C-t> :NERDTreeToggle<CR> " Toggle NERDTree
nnoremap <C-r> :NERDTreeRefreshRoot<CR> " Refresh NERDTree

nnoremap <C-s> :w<CR> " Save
inoremap <C-s> <Esc>:w<CR> " Save and return to normal mode

nnoremap <Leader>b :ls<CR>:buffer<Space> " Open a list of buffers and open the selected one

" Enter key to confirm completion
inoremap <expr> <cr> coc#pum#visible() ? coc#pum#confirm() : "\<CR>"
" Enter key to select the first and accept it if none are selected
inoremap <silent><expr> <cr> coc#pum#visible() ? coc#_select_confirm() : "\<C-g>u\<CR>"

" Tab/Split nav
noremap <silent> <C-h> :tabprevious<CR> " Previous tab
noremap <silent> <C-l> :tabnext<CR> " Next tab

noremap <silent> <A-h> :wincmd h<CR> " Previous window
noremap <silent> <A-l> :wincmd l<CR> " Next window

" Newline noninsert commands
nnoremap <Leader>o o<Esc> " Newline without entering insert mode
nnoremap <Leader><S-o> O<Esc> " Newline above without entering insert mode

nnoremap r :redo<CR> " Redo

nnoremap <C-_> :Commentary<CR> " Comment line
```
