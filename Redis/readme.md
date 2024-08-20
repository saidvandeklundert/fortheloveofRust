

## Redis basic commands

```
redis-cli   # start CLI
KEYS *      # list all available keys


###
# create backup
###

SAVE
config get dir
# go to the dir and fetch the .rdb file


##
# Use backup
##
# stop Redis server
/etc/init.d/redis-server stop
# copy backup to `config get dir`
# start Redis again
/etc/init.d/redis-server start

# in Docker:
docker run -d --hostname=9d04a6d581cd --name redis_sonic  -v /tmp/dump.rdb:/data/dump.rdb -p 6379:6379 redis:latest 

docker cp dump.rdb 9d04a6d581cd6b7a73916905e02864eab50052c3c0e78d1f6d87fae585b11f86:/data/dump.rdb

docker run --hostname=9d04a6d581cd --env=PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin --env=GOSU_VERSION=1.17 --env=REDIS_VERSION=7.2.5 --env=REDIS_DOWNLOAD_URL=http://download.redis.io/releases/redis-7.2.5.tar.gz --env=REDIS_DOWNLOAD_SHA=5981179706f8391f03be91d951acafaeda91af7fac56beffb2701963103e423d --volume=/data --workdir=/data -p 6379:6379 --restart=no --runtime=runc -d redis

```