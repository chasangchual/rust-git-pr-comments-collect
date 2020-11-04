docker volume create pgdata

docker run -d -p (외부접속포트:내부접속포트) --name (container 이름) -it --rm -v pgdata:(data경로) -e POSTGRES_PASSWORD=(비밀번호) (postgres 이미지)

docker run --name git-pr-comments-collect -p 5678:5432 -v postgresql:/var/lib/postgresql/pgdata --restart=always -e POSTGRES_PASSWORD=Abc12345! -d postgres