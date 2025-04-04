const imageUrls = [
    "https://my-photo-bucket-cp.s3.amazonaws.com/processed/medium_matcha.jpg",
    "https://my-photo-bucket-cp.s3.amazonaws.com/processed/thumb_matcha.jpg",
    "https://my-photo-bucket-cp.s3.amazonaws.com/processed/thumb_photo3.jpg"
  ];
  
  const gallery = document.getElementById("gallery");
  
  imageUrls.forEach(url => {
    const a = document.createElement("a");
    a.href = url;
    a.download = ""; // this triggers the browser's download behavior
  
    const img = document.createElement("img");
    img.src = url;
    img.alt = "Processed image";
  
    a.appendChild(img);
    gallery.appendChild(a);
  });