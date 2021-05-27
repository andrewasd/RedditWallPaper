const fetch = require("node-fetch");
const fs = require("fs");
const { exec } = require("child_process");
const { argv } = require("process");

const config = JSON.parse(fs.readFileSync(argv[2]+"/redditConfig.json"));

const sortType = config.sortby;
const subs = config.subreddits;
const removeoldpics = config.deleteOldPics;


fetchRandomUrl();


async function fetchRandomUrl () 
{
  await deleteOldPics();
  const randomSub = subs[getRandomInteger(0,subs.length-1)];

  const url = createUrl(randomSub,sortType);
   
    fetch(url, { method: "Get"})
    .then((res) => res.json())
    .then((json) => {

      const total = json.data.children.length;
      const index = Math.floor(Math.random() * total);

      const imageLink = json.data.children[index].data.url;

      if(isPicture(imageLink))
      {
        console.log(imageLink);
        process.exit();
      }  

        fetchRandomUrl();  
    }); 
}


function deleteOldPics()
{
  exec("rm -rf ./pics/*");
}

function createUrl(subRedditName,sortOrder)
{
  return "https://www.reddit.com/r/" + subRedditName + "/" + sortOrder + ".json";
}

function getRandomInteger(min, max)
{
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

function isPicture(url) {
  if (url.endsWith(".png") || url.endsWith(".jpg") || url.endsWith(".jpeg"))
    return true;
}

