FROM  node:10-alpine


WORKDIR /usr/app
COPY pachager.json yarn.lock ./      

RUN yarn 

COPY . .

EXPOSE 3000
CMD ["yarn", "start"]
