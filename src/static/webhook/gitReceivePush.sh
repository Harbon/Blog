#!/bin/sh
WEB_PATH='~/Desktop/Blog_file/'
WEB_USER='Harbon'
WEB_USERGROUP='Harbon'


echo "Start deployment............"
cd $WEB_PATH
echo "pulling source code..."
git reset --hard origin/master
git clean -df
git pull origin master
if [$? -eq 1];then
    echo 'git pull blog error'
  else
    git checkout master
    echo "changing permissions..."
    chown -R $WEB_USER:$WEB_USERGROUP $WEB_PATH
    echo "begin build blog"
    echo $PWD
    cargo build
    if [ -e ./target/debug/blog ];then
      set blog_pid = lsof -i:3000|tail -n 1|cut -d ' ' -f 5
      echo "start kill pid $blog_pid"
      kill $blog_pid
      cd target/debug/
      ./blog &
      if [ $? -eq 0 ];then
        echo 'Your blog has been updated and run successfully'
      else
        echo 'run blog error please manipulate it'
      fi
    else
      echo 'cargo build blog error no target directory'
    fi


fi
