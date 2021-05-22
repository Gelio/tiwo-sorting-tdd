# Insertion sort developed with TDD

This is a solution to an assignment for the TiWO (_Testowanie i Weryfikacja
Oprogramowania_, Testing and Verifying Software) course.

The aim was to implement
[the Insertion Sort algorithm](https://www.tutorialspoint.com/data_structures_algorithms/insertion_sort_algorithm.htm)
with the _TDD_ (Test-Driven Development) approach.

The algorithm is implemented in [Rust](https://www.rust-lang.org/).

This project is a library, meaning it does not offer an executable binary.

## Requirements

1. Rust installed
2. Cargo installed

## Executing tests

Run the following command to execute tests:

```sh
cargo test
```

## Generating reports

Install [pandoc](https://pandoc.org/), pdflatex and texlive:

```sh
sudo apt install pandoc pdflatex texlive texlive-lang-polish
```

Then, to generate the reports, run:

```sh
cd reports
./generate.sh
```

This will generate PDFs from all Markdown reports.

## Author

The author of the project is Grzegorz Rozdzialik.
