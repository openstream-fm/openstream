cargo build --release &&
cd front/app && npm i && npm run build &&
cd ../server && npm i && npm run build