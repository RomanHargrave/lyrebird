# *Building Lyrebird*

This is the first time I have really done anything with Rust. I find
that I learn best by solving problems with a tool I have never used
before, as opposed to doing small, isolated exercises such as those on
*exercism.io*. Knowing this about myself, I found this an excellent
occasion to put Rust to work, and to learn by doing.

## What did you learn?

I certainly became more closely acquainted with Rust than I had
previously been. Certain patterns that are common in Rust (such as the
combinatorial approach to emptiness and error handling) I had already
been familiar with from other languages, such as Scala. The
borrow-checker was new (and is much disdained, so I hear, by
newcomers), but anticipated, and I had no issue "thinking like the
compiler". There were incidents where I did things such as try to move
members of a structure, but the tooling feedback cycle was quick
(thanks rls, and to some degree JetBrains) and I corrected those
issues as soon as I noticed them.

Amusingly, the first thing I worked on was username discovery. This is
a surprisingly complex problem, at least on Unix(like) systems.
Available options range from simplest to most complex and,
incidentally, least reliable to most reliable in the same order. The
first of these is to check environment variables (`USER` or
`LOGNAME`), which may not exist and could be changed; the next is
[`getlogin(3)`](https://man7.org/linux/man-pages/man3/getlogin.3.html),
which reads `/var/run/utmp` on most platforms and therefor relies upon
the integrity of the file (just read
[`utmp(5)`](https://man7.org/linux/man-pages/man5/utmp.5.html)) and
that it exists in the first place (and it apparently may not);
finally, we are left with the `getpwnam(3)` family of functions, which
actually try to look up a given username or UID via the NSS libraries
that have been selected for the user (see
[`nss(5)`](https://www.man7.org/linux/man-pages/man5/nss.5.html) and
[`nsswitch.conf(5)`](https://www.man7.org/linux/man-pages/man5/nsswitch.conf.5.html)).

Reviewing the `getpwuid_r()` interface, which actually copies data
into a buffer of your choosing instead of using a shared buffer, I was
initially somewhat intimidated at the idea of having to call this
thing from Rust. *Just look at the interface:* it is completely alien
to Rusts approach. Fortunately, some reading of the
[Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html) and the
[Reference](https://doc.rust-lang.org/reference/unsafety.html) proved
fruitful and with some experimentation I was able to arrive at a
robust solution. Overall, implementing this was a great ramp-up
excercise.


## Rationale: Why Rust?

Certainly, desire to apply new tools is not - and should not
necessarily serve as - the sole reason to make decisions about how you
will implement something. In the realm of collaborative (thus, also
commercial) software engineering, making design decisions based only
upon the "shiny factor" holds great potential to create technical debt
down the line for a number of reasons; thus, *what are the practical
reasons for this decision*?

Originally, I considered that the best way to implement an easily
packaged and distributed tool would be to use a language that had
straightforward build tooling and relatively good support for features
that reduce the complexity of targeting many platforms. Generally,
this means that the language meets the following criteria:

1. Platform-specific functionality is abstracted into a standard API
   across all platforms.
2. Where platform-specific functionality must be implemented, it is
   easily separated and can be readily identified and reasoned about.
3. The language is mature (enough) on all target platforms.

Fortunately, most reasonable languages satisfy these requirements. One
has the choice of Python, Ruby, C, C++, Go, Rust, Kotlin MP, and more;
however, the above qualifiers are concerned only with platform
support, and there are further criteria that I feel are important,

1. Dependency management should be straightforward, ideally a
   first-class citizen and part of the tooling.
2. An approachable build system should be available, preferably having
   a high level of integration with dependency management.
3. It should be possible to deliver the application as a relatively
   portable package. This can help to simplify containerization and
   testing pipelines.
   
Python has improved over the years where the above items are
applicable, but still struggles with dependency management compared to
the other options - venv helps ease some of that pain, and PyInstaller
provides a route to a portable executable; however, none of this feels
"first class", and more time must be spent on tooling as a result.

Ruby, via Gems and the second-class but very well integrated *bundler*
has nailed it in the project-dependency management department;
however, stand-alone packaging has never really been a huge need or
concern in the domains where Ruby is used (at least outside of Japan),
and as such it doesn't pass muster for this project.

C & C++ are approaching what's needed here. With Conan and CMake, they
can certainly manage to tick all the boxes; however, Conan and CMake
are still very much borne of the "bazaar" philosophy that is
predominant in the C/C++ community. This is not necessarily a bad
thing, and Conan is a major improvement and huge boon for C/C++
development. Nonetheless, it's not en-suite.

Go is pleasant enough, and is worth considering for fact that it's
apparently become quite popular with malware developers. It has
cathedral-style tooling, spits out statically linked binaries, and has
the conditional compilation features that are necessary for certain
multiplatform work.

Rust is promising in this area. Cargo, though far from a strict
requirement for building Rust software, seamlessly integrates
packaging, dependency management, and compilation in a straightforward
fashion. Documentation for Cargo and Rust is good, even when you need
to do [uncommon things](./src/platform/unix.rs). And what's more, like
Go, it spits out binaries with minimal dynamic linker dependencies (in
most cases).

[Kotlin Native](https://kotlinlang.org/docs/multiplatform.html) simply
gets honorable mention. It's extremely promising, but it's still in
development, and I suspect the native tooling and ecosystem is a
littly iffy at present.

This leaves the choice between Go and Rust. I had been playing with Go
a little bit, and could see its appeal for service development, and
possibly even systems development; however, I felt that Rust might
have more to offer in the latter area. For one, overhead when calling
C-ABI code is much lower, and the Go ecosystem seems to be largely
focused on service development.
