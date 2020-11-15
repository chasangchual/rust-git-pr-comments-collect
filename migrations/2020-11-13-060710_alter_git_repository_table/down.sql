-- This file should undo anything in `up.sql`
ALTER TABLE public.git_repository DROP COLUMN "number";
ALTER TABLE public.git_repository DROP COLUMN full_name;
ALTER TABLE public.git_repository DROP COLUMN private;
ALTER TABLE public.git_repository DROP COLUMN description;
ALTER TABLE public.git_repository DROP COLUMN "language";
ALTER TABLE public.git_repository DROP COLUMN url;
ALTER TABLE public.git_repository DROP COLUMN "size";
ALTER TABLE public.git_repository DROP COLUMN stargazers_count;
ALTER TABLE public.git_repository DROP COLUMN watchers_count;
ALTER TABLE public.git_repository DROP COLUMN forks_count;
ALTER TABLE public.git_repository DROP COLUMN open_issues_count;
ALTER TABLE public.git_repository DROP COLUMN open_issues;
ALTER TABLE public.git_repository DROP COLUMN watchers;
ALTER TABLE public.git_repository DROP COLUMN subscribers_count;
