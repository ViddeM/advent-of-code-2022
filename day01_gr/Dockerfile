FROM rust:1.65
WORKDIR /usr/src/aoc
COPY . .
RUN cargo install vm_lang

COPY input.txt .

CMD ["vm_lang", "prog.gr"]