+++
title = "Shell completions from nix and brew"
+++

One thing that I noticed when I installed packages from `nix` and `homebrew`
was the fact that I was missing `zsh` completions! I was looking at
`nix-daemon.sh` and it does add some files installed to `XDG_DATA_DIRS` but it
doesn't do anything to your shells regarding completions. Same with `homebrew`.

To resolve, add this before your `compinit` in your `.zshrc`:
```bash
fpath=(~/nix-profile/share/zsh/site-functions $fpath)
if type >> /dev/null; then
    HOMEBREW_PREFIX=$(brew --prefix)
    fpath=($HOMEBREW_PREFIX/share/zsh/site-functions $fpath)
fi
```

### Man

I was curious why my `man` pages worked even though they don't seem to be set
in `MANPATH`.

I realized that `manpath` also searches for man pages by looking in your `PATH`
and sees if a pathname ends in `/bin` and looks for `/share/man` or `/man` in
the same pathname. In Linux (it also uses `manpath`), there is also
`man_db.conf` which does something similar with `MANPATH_MAP /bin
/usr/share/man`, so it doesn't have to look at `PATH`.
