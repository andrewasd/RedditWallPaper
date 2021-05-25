const fetch = require("node-fetch");

const sortType = process.argv.pop();
const subs = process.argv.slice(2);

const fetchNow = function () 
{
  const randomSub = subs[getRandomInteger(0,subs.length-1)];

  const url = createUrl(randomSub,sortType);

  fetch(url, { method: "Get"})
    .then((res) => res.json())
    .then((json) => {
      const total = json.data.children.length;
      const index = Math.floor(Math.random() * total);

      const imageLink = json.data.children[index].data.url;

      if (
        imageLink.endsWith(".png") ||
        imageLink.endsWith(".jpg") ||
        imageLink.endsWith(".jpeg")
      )
      {
        console.log(imageLink);
        process.exit();
      }
    
      
        fetchNow();
      
       

      
    });
}

fetchNow();

function createUrl(subRedditName,sortOrder)
{
  return "https://www.reddit.com/r/" + subRedditName + "/" + sortOrder + ".json";
}

function getRandomInteger(min, max)
{
    return Math.floor(Math.random() * (max - min + 1)) + min;
}


