+++
title = "Doing Weblab Again"
+++

This month, I participated in [weblab](https://weblab.mit.edu) with a couple of
my friends to make [enuTyping](https://enutyping.timmy.moe). This was my second
time doing weblab; I did weblab during the start of 2019, but to be honest, I
didn't really learn much other than writing React (it was class-based back
then). I have much more experience with web development now, so I am here to
write down things that I witnessed when participating again.

### Configuring React from scratch

All React code that I have written before didn't start from scratch. Projects
were already configured or configured with
[create-react-app](https://create-react-app.dev/), so I didn't have to worry
about setup. The weblab staff provides a skeleton that almost everyone starts
from, but our team decided to start from scratch. I finally learned `babel` and
`webpack` in depth. A lot of the details about building and deploying React
apps started making sense. One good thing that came from doing this was
integrating Typescript, something that the weblab staff taught this year but
never integrated to the project skeleton, to our project. Setting stuff from
scratch was pretty painful and tedious (our team spend quite a bit on asset
management), so I can see why skeletons and tools that help you get started exist.

### Thinking functionally

I decided to dabble in learning Haskell last year, and it has made me think
about writing programs differently, but I almost never apply it. However,
writing React again changed my perspective, and it has made me understand some
of React's design choices. Functions are pure (the same props makes the same
JSX) and we avoid mutating data (we set it and optionally use previous values).
It seems useful to apply functional programming in an asynchronous environment.
I am not saying you should learn about functional programming right now. In
fact, writing React probably makes you understand functional programming better
if you are ever exposed to it; I just didn't spend time to understand React
  when I first touched it.

### Reproducible environments

"Works on my machine." Starting a project from scratch and working with other
people makes me appreciate tools that allow reproducible environments. It was
not a huge mess for such a small project and team, but it was still quite
agonizing to deal with. Also having different versions of the same software and
having a shell environment with the exact dependencies you desire makes me want
to try Nix again.
