const redis = require('redis');
const host = 'redis';
const port = '6379';
const client = redis.createClient(port, host);

client.on('connect', function(){
  console.log('==> Conexao com o REDIS OK!')
})

client.on('error', function(){
  console.log('==> Falha na conexaO  COM O REDIS OK!')
})

client.on('error', function (err){
  console.log('==> falha na conexao com o REDIS: ' + err);
});

module.exports =client;

