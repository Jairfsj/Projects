const mongoose          = require('../conf/db');
const Schem             = mongoose.Schema;


const emailSchema  = new Schema({
  remetente:{
    type:String

  },
  destinatario:{
    type:String

  },
  assunto:{
    type:String  

  },
  texto:{
    type:String  
  }
});

const emailModel = mongoose.model('email', emailSchema);
module.exports = emailModel;
