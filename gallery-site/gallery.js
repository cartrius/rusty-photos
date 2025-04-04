const imageUrls = [
    "https://my-photo-bucket-cp.s3.amazonaws.com/processed/thumb_photo1.jpg",
    "https://my-photo-bucket-cp.s3.amazonaws.com/processed/thumb_photo2.jpg",
    "https://my-photo-bucket-cp.s3.amazonaws.com/processed/thumb_photo3.jpg"
  ];
  
  const gallery = document.getElementById("gallery");
  
  imageUrls.forEach(url => {
    const img = document.createElement("img");
    img.src = url;
    img.alt = "Processed image";
    gallery.appendChild(img);
  });
  