const mongoose = require("mangoose");
const host = "mongo";
const port = "27017";

mangoose.connect(`mongodb://${host}:${port}/email`).then(connect => {
  console.log("==> conexao com o Mongo OK!");

}).catch(err=> {
    
    console.log("===> falha na conexao com o MongoDB!");
  });

module.exports = mongoose;
