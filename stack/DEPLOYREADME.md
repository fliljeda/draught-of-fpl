First time on a server, from draught-of-fpl/stack:
1. cp .env.example .env
2. cp acme.json.example acme.json
3. chmod 600 acme.json

To deploy:
1. Build docker image as dof:latest
2. docker save dof:latest -o /tmp/dof.tar
3. scp /tmp/dof.tar fpl:/tmp/dof.tar
4. ssh fpl
5. docker image load -i /tmp/dof.tar
6. git pull
7. cd draught-of-fpl/stack
8. Update .env
9. docker compose up -d --force-recreate dof
