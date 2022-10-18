+++
title = "Neovim Efficiency"
+++

Recently I have been recording myself writing code to see what bottlenecks I am
having when writing code. To be frank, I didn't go back and watch the videos in
their entirety. The action is quite embarrassing and a bit too tedious.
However, when recording myself I was more conscious of what was slowing me
down, so I just jotted them down. Obviously, I spend most of my time thinking
when coding, but that's much harder to be more efficient at than just improving
the speed of mindless tasks in my text editor (Neovim). This allows me to spend
more time thinking! Here are some editor bottlenecks that I noticed and what I
learned to resolve them.

### Deleting backwards in insert mode

Usually when editing text, I am already in insert mode and I have made a
mistake for the current "word" that I am typing. I usually just delete the
current word I am on and try to retype correctly. Before, I had to leave insert
mode and then delete my current word and then enter insert mode again (or use
my terrible habit of just spamming the delete key). My keystrokes for that
would be `<Esc> + d + i + w + i` (`d + b` doesn't work as my cursor would not
be after my word so it would leave the last character intact). But you can just
delete the word before the cursor in insert mode with just `<Ctrl>-W`. This
forces me to use two fingers at once, but I would have a lot less keystrokes -
a worthy tradeoff for me.

### Moving from a buffer to a window

Sometimes I am editing two files individually in buffers, but then I would want
to view them side-by-side. Before, I would just `:vs` and then `:e otherfile`,
but you can also just `:vs otherfile`. Also, if your `otherfile` is the
alternate buffer (the last file that was in the current window), you can also
just `:vs #`. Another neat trick is that you quickly switch between alternate
buffers in normal mode with `<Ctrl>-6`. There's also
[telescope.nvim](https://github.com/nvim-telescope/telescope.nvim), where you
can vertically split from the file selector with the default keymap `<Ctrl>-v`.

### Pasting from the previous yank

This is probably the most common bottleneck that I have using Neovim. I would
need to copy text from place A to replace text at place B (or something very
similar). The problem is that when I am deleting the text at B, I would be
replacing the yanked text from A. Therefore, I would just paste A's text
somewhere, delete B's text, cut A's text, paste A's text to where I deleted B's
text. There are three ways to do this better.

1. Use named registers. Just yank A's text to register `"a` with `"ay` and then
   paste from it using `"ap`.
2. When deleting B's text, put it in the black hole register `"_` with `"_d` (I
   guess you put it in some other register that you will never use).
3. Using numbered registers. `"0` would contain the most recent yank and `"1`
   would contain the most recent delete/change. The other numbered registers
   would be the previous deletes/changes in order. In the example, I
   never yanked twice so `"0` would still contain A's text, so I can delete B's
   text and then paste from `"0`.

### Copying text from remote Neovim session to my local clipboard

My Neovim configuration has `mouse=a` (I think newer versions of Neovim now
have most modes enabled with mouse support by default), so highlighting with my
mouse and then copying from my terminal emulator doesn't work as it would just
bring my Neovim to visual mode. There's also the existence of line numbers,
unless I turn them off. I can also use tmux's copy mode to select the text that
I want, but the line numbers will still exist. I can't find any other solution
without installing a plugin in my remote vim session. I use
[nvim-osc52](https://github.com/ojroques/nvim-osc52), which allows your remote
Neovim session to copy to your local clipboard using the ANSI OSC52 sequence.
To use it with tmux, be sure to have `set-clipboard` to `on`.

### Aren't you wasting more time by configuring?

Yes, I wasted my time to learn how to waste less time. But these resolutions
that I just listed are actually worth knowing in my opinion. These bottlenecks
happen pretty frequently and they didn't take that much effort to learn how to
resolve them. In retrospect, recording and viewing my recording would have
probably been a huge time waster, but without any direct actions taken, I
wouldn't have been conscious of what was slowing me down. 
