#!/bin/sh
WEB_PATH='/Home/Harbon/blog_file'
WEB_USER='Harbon'
WEB_USERGROUP='Harbon'


echo "Start deployment............"
cd $WEB_PATH
echo "pulling source code..."
git reset --hard origin/master
git clean -df
git pull origin master
if [$? -eq 1];then
    echo 'git pull blog error' | mail -s 'server robot report' qq@harbon.link
  else
    git checkout master
    chown -R $WEB_USER:$WEB_USERGROUP $WEB_PATH
    cargo build
    if [ -e ./target/debug/blog ];then
      blog_pid=`lsof -i:3000|tail -n 1|cut -d ' ' -f 5`
      kill $blog_pid
      if [ $? -eq 1];then
        echo 'kill blog_pid error' | mail -s 'server robot report' qq@harbon.link
      fi
      cd target/debug/
      ./blog &
      if [ $? -eq 0 ];then
        echo 'Your blog has been updated and run successfully' | mail -s 'server robot report' qq@harbon.link
      else
        echo 'run blog error please manipulate it' | mail -s 'server robot report' qq@harbon.link
      fi
    else
      echo 'cargo build blog error no target directory' | mail -s 'server robot report' qq@harbon.link
    fi


fi
