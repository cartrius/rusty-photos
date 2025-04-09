document.addEventListener("DOMContentLoaded", () => {
  loadGallery();
});

async function loadGallery() {
try {
  // Fetch JSON from API endpoint
  const response = await fetch("http://localhost:3000/list-images");
  if (!response.ok) {
    throw new Error("Failed to fetch image list");
  }
  console.log("fetched")
  // Respresented as { images: [...] }
  const data = await response.json(); 
  const imageUrls = data.images || [];

  const gallery = document.getElementById("gallery");
  // Clear existing content
  gallery.innerHTML = "";

  imageUrls.forEach(url => {
    console.log("test")
    const a = document.createElement("a");
    a.href = url;
    a.download = "";

    const img = document.createElement("img");
    img.src = url;
    img.alt = "Processed image";

    a.appendChild(img);
    gallery.appendChild(a);
  });
} catch (err) {
  console.error("Error loading gallery:", err);
}
}