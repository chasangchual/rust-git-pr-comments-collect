-- Your SQL goes here
ALTER TABLE public.git_repository ADD "number" int8 NULL;
ALTER TABLE public.git_repository ADD full_name varchar NULL;
ALTER TABLE public.git_repository ADD private boolean NULL;
ALTER TABLE public.git_repository ADD description text NULL;
ALTER TABLE public.git_repository ADD "language" varchar NULL;
ALTER TABLE public.git_repository ADD url varchar NULL;
ALTER TABLE public.git_repository ADD "size" int4 NULL;
ALTER TABLE public.git_repository ADD stargazers_count int4 NULL;
ALTER TABLE public.git_repository ADD watchers_count int4 NULL;
ALTER TABLE public.git_repository ADD forks_count int4 NULL;
ALTER TABLE public.git_repository ADD open_issues_count int4 NULL;
ALTER TABLE public.git_repository ADD open_issues int4 NULL;
ALTER TABLE public.git_repository ADD watchers int4 NULL;
ALTER TABLE public.git_repository ADD subscribers_count int4 NULL;
