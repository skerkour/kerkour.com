```tex
\usepackage{fancyvrb,newverbs,xcolor}

\definecolor{Light}{HTML}{F4F4F4}

\let\oldtexttt\texttt
\renewcommand{\texttt}[1]{
  \colorbox{Light}{\oldtexttt{#1}}
}

\renewenvironment{Shaded} {\begin{snugshade}\small} {\end{snugshade}}
```

> I have such snippets for all different sizes: huge, LARGE, Large, large, normalsize, small, footnotesize, scriptsize, tiny.
