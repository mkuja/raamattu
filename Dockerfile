FROM rust:1.81-bookworm AS build
WORKDIR build
COPY . .
RUN cargo build --release

FROM debian:bookworm AS deploy
WORKDIR app
COPY --from=build /build/target/release/raamattu .
COPY --from=build /build/db db
COPY --from=build /build/static static
EXPOSE 3000
CMD /app/raamattu

# vim: et ts=4 sw=4
