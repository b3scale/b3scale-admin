import torch
from diffusers import StableDiffusionPipeline
import re
import os

model_id = "CompVis/stable-diffusion-v1-4"
device = "cuda"

pipe = StableDiffusionPipeline.from_pretrained(model_id, torch_dtype=torch.float16)
pipe = pipe.to(device)

def read_prompts_from_file():
    """Read prompts from STABLE_DIFFUSION_PROMPTS.md"""
    try:
        with open('STABLE_DIFFUSION_PROMPTS.md', 'r') as f:
            content = f.read()
        
        # Extract prompts from code blocks
        prompts = []
        pattern = r'```\n(.*?)\n```'
        matches = re.findall(pattern, content, re.DOTALL)
        
        for match in matches:
            if len(match.strip()) > 20:  # Only real prompts, not short code
                prompts.append(match.strip())
        
        return prompts
    except:
        return ["cyberpunk neon terminal background, matrix style, ultra detailed"]

# Read prompts from file
prompts = read_prompts_from_file()
print(f"Found {len(prompts)} prompts! 🔥")

# Generate images for each prompt
for i, prompt in enumerate(prompts):
    print(f"Generating image {i+1}/{len(prompts)}...")
    print(f"Prompt: {prompt[:80]}...")
    
    image = pipe(prompt).images[0]
    image.save(f"cyber_graphic_{i+1}.png")
    print(f"Saved: cyber_graphic_{i+1}.png ✨")

print("🎯 All cyber graphics generated choom! 💚💖")

