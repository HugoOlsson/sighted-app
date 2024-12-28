from sighted import sighted_setup, sighted_wait, sighted, text
from points3d_demos import (
    room_plot,
    animated_torus_plot,
    mobius_strip_plot,
    animated_spherical_grid_plot,
    animated_particle_swarm_plot,
    animated_fractal_cloud_plot,
    enhanced_room_plot
)

from lines_demos import (
    generate_lines,
    generate_damped_oscillations,
    generate_fourier_square_wave,
    generate_sine_cosine_waves,
    generate_spiral_waves,
    generate_harmonic_waves,
    generate_circular_dance,
    generate_fractal_flow,
    generate_quantum_waves

)

from grid_demos import (
    heat_maps,
    interference_grid,
    cellular_grid,
    fractal_grid,
    vector_field_grid,
)

def run_text_tests():
    # Pre-define the ranges for better readability
    outer_range = range(2000)
    inner_range = range(100)
    
    # Define messages outside the loop to avoid repeated string creation
    messages = [f"Detta är mitt fina meddelande som jag printar i terminalen (very good) {i} " for i in range(1, 8)]
    
    for _ in outer_range:
        # Using enumerate to get both index and message
        for idx, msg in enumerate(messages, 1):
            for i in inner_range:
                #print(msg + str(i))
                sighted(text(msg + str(i)), str(idx))

                print("Here is my good yapping that is very nice hihi", i)

def main():
    sighted_setup("test-project-py", True)
    
    # Run your choice of visualization

    """
    generate_lines()
    generate_damped_oscillations()
    generate_fourier_square_wave()
    generate_sine_cosine_waves()
    generate_spiral_waves()
    generate_harmonic_waves()
    generate_circular_dance()
    generate_fractal_flow()
    generate_quantum_waves()

    heat_maps()
    interference_grid()
    cellular_grid()
    fractal_grid()
    vector_field_grid()

    """

    room_plot()
    animated_torus_plot()
    mobius_strip_plot()
    animated_spherical_grid_plot()
    animated_particle_swarm_plot()
    animated_fractal_cloud_plot()
    enhanced_room_plot()
   

    run_text_tests()
    
    sighted_wait()

if __name__ == "__main__":
    main()



