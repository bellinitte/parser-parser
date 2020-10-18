FROM jdrouet/rust-nightly

# wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# nodejs and npm
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get install -y nodejs

# serve
RUN npm install -g serve

WORKDIR /usr/src/parser-parser
COPY . .

RUN npm ci
RUN npm run build

EXPOSE 8080

CMD ["serve", "-s", "-l", "8080", "dist"]
