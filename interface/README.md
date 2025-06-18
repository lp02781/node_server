# Installation
```
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs
node -v
npm -v
```

```
npm init -y
npm install next react react-dom
```

```
package.json
---
{
  "name": "js_fullstack",
  "version": "1.0.0",
  "description": "Fullstack app using Next.js and Node.js",
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start"
  },
  "keywords": [],
  "author": "",
  "license": "ISC",
  "dependencies": {
    "next": "^15.3.3",
    "react": "^19.1.0",
    "react-dom": "^19.1.0"
  }
}
```

```
npm run dev
```

