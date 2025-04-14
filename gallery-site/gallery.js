document.addEventListener("DOMContentLoaded", () => {
  loadGallery();

  const uploadBtn = document.getElementById("uploadButton");
  const fileInput = document.getElementById("fileInput");

  // 1) When the user clicks the button, "click" the hidden file input
  uploadBtn.addEventListener("click", () => {
    fileInput.click();
  });

  // 2) When the user actually picks a file, do the upload
  fileInput.addEventListener("change", () => {
    if (fileInput.files[0]) {
      uploadImage(fileInput.files[0]);
      // Clear fileInput so subsequent clicks re-trigger:
    }
  });
});

async function uploadImage(file) {
  try {
    const key = `uploads/${file.name}`;

    const allowedTypes = ["image/png", "image/jpeg"];
    if (!allowedTypes.includes(file.type)) {
    alert("Only PNG or JPG files allowed!");
    return;
  }

    // Get a pre-signed URL
    const response = await fetch(
      `http://localhost:3000/get-upload-url?key=${encodeURIComponent(key)}`
    );
    if (!response.ok) {
      alert("Failed to get pre-signed URL");
      return;
    }
    const presignedUrl = await response.text();

    // PUT the file directly to S3
    const putResp = await fetch(presignedUrl, {
      method: "PUT",
      body: file,
      // headers: { "Content-Type": file.type }
    });
    if (!putResp.ok) {
      alert("Upload failed!");
      return;
    }

    alert("Upload succeeded!");
    loadGallery();
  } catch (error) {
    console.error("Error uploading image:", error);
    alert("Error uploading image.");
  }
}

// Call /list-images
async function loadGallery() {
  try {
    const response = await fetch("http://localhost:3000/list-images");
    if (!response.ok) throw new Error("Failed to fetch image list");

    const data = await response.json();
    const imageUrls = data.images || [];

    const gallery = document.getElementById("gallery");
    gallery.innerHTML = "";

    imageUrls.forEach(url => {
      const container = document.createElement("div");
      container.classList.add("photo-item");

      const img = document.createElement("img");
      img.src = url;
      img.alt = "Processed image";
      container.appendChild(img);

      // Create a "Delete" button
      const delBtn = document.createElement("button");
      delBtn.classList.add("delete-btn");
      delBtn.innerHTML = "🗑️";
      delBtn.title = "Delete Photo";
      delBtn.addEventListener("click", () => {
        handleDelete(url);
      });
      container.appendChild(delBtn);
      gallery.appendChild(container);
    });
  } catch (err) {
    console.error("Error loading gallery:", err);
  }
}

async function handleDelete(processedUrl) {
  // Extract the filename, e.g. "thumb_myphoto.jpg"
  const parts = processedUrl.split("/");
  const processedFilename = parts[parts.length - 1]; 

  // Remove "thumb_" or "medium_" if present
  let baseFilename = processedFilename.replace("thumb_", "");
  baseFilename = baseFilename.replace("medium_", "");

  // e.g. "uploads/myphoto.jpg"
  const originalKey = `uploads/${baseFilename}`;

  // Call the DELETE endpoint
  const deleteUrl = `http://localhost:3000/photos/${encodeURIComponent(originalKey)}`;
  try {
    const resp = await fetch(deleteUrl, { method: "DELETE" });
    if (resp.ok) {
      alert(`Deleted ${originalKey} successfully!`);
      // Re-load the gallery
      loadGallery();
    } else {
      alert(`Failed to delete ${originalKey}.`);
    }
  } catch (error) {
    console.error("Error deleting photo:", error);
  }
}