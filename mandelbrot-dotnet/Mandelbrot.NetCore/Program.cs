using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Drawing;
using System.Drawing.Imaging;
using System.Numerics;
using System.Runtime.CompilerServices;

namespace Mandelbrot.NetCore
{
    class Program
    {
        [DllImport("shlwapi.dll")]
        static extern int ColorHLSToRGB(int h, int l, int s);

        // Mandelbrot settings
        private const int MaxIterations = 1000;
        private const int MaxRadius = 2;

        // Drawing settings
        private const int PixelsPerUnit = 2000;
        private const int WindowXMin = -2;
        private const int WindowXMax = 1;
        private const int WindowYMin = -1;
        private const int WindowYMax = 1;
        private const int ColorHueFactor = 5; // higher factor = more different colors

        private const int WidthInUnits = WindowXMax - WindowXMin;
        private const int HeightInUnits = WindowYMax - WindowYMin;
        private const int WidthInPixels = WidthInUnits * PixelsPerUnit;
        private const int HeightInPixels = HeightInUnits * PixelsPerUnit;
        private const int TotalPixels = WidthInPixels * HeightInPixels;

        private static IDictionary<uint, SolidBrush> ColorMap;

        static void Main(string[] args)
        {
            //PrintMandelbrotSetToConsole();
            CreateMandelbrotSetBitmap();
            Console.Read();
        }

        static void PrintMandelbrotSetToConsole()
        {
            for (var y = 1.0d; y >= -1.0d; y -= 0.1)
            {
                for (var x = -2.0d; x <= 1.0d; x += 0.03)
                {
                    Console.Write(BelongsToMandelbrotSet(new Complex(x, y), out uint iterations) ? "x" : " ");
                }
                Console.Write("\n");
            }
        }

        static void CreateMandelbrotSetBitmap()
        {
            ColorMap = new Dictionary<uint, SolidBrush>();
            var bitmap = new Bitmap(WidthInPixels, HeightInPixels, PixelFormat.Format24bppRgb);

            using (var graphics = Graphics.FromImage(bitmap))
            {
                graphics.Clear(Color.Black);
                // draw mandelbrot set
                for (var y = 0; y < HeightInPixels; y++)
                {
                    for (var x = 0; x < WidthInPixels; x++)
                    {
                        var real = (double)x / PixelsPerUnit + WindowXMin;
                        var imaginary = (double)-y / PixelsPerUnit + WindowYMax;
                        if (!BelongsToMandelbrotSet(new Complex(real, imaginary), out uint iterations))
                        {
                            var brush = GetMandelbrotColor(iterations);
                            graphics.FillRectangle(brush, x, y, 1, 1);
                        }
                    }
                }
                graphics.Flush();
            }
            bitmap.Save("mandelbrot.bmp");
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        static SolidBrush GetMandelbrotColor(uint iterations)
        {
            if (!ColorMap.ContainsKey(iterations))
            {
                //var colorRgb = ColorHLSToRGB((int)((double)iterations * ColorHueFactor % 240), 120, 120);
                var hue = ((double)iterations * ColorHueFactor % 360) / 360;
                var rgb = HSL2RGB(hue, 0.5, 0.5);
                var color = Color.FromArgb(rgb.R, rgb.G, rgb.B);
                //var color = ColorTranslator.FromWin32(colorRgb);
                var brush = new SolidBrush(color);
                ColorMap.Add(iterations, brush);
                return brush;
            }
            return ColorMap[iterations];
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        static bool BelongsToMandelbrotSet(Complex c, out uint iterations)
        {
            iterations = 0;

            var result = c;

            while (iterations < MaxIterations)
            {
                iterations++;
                result = Mandelbrot(c, result);
                if (result.Magnitude > MaxRadius)
                    return false;
            }

            return true;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        static Complex Mandelbrot(Complex initial, Complex current)
        {
            return Complex.Pow(current, 2) + initial;
        }

        // Given H,S,L in range of 0-1
        // Returns a Color (RGB struct) in range of 0-255
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        static ColorRGB HSL2RGB(double h, double sl, double l)
        {
            double v;
            double r, g, b;

            r = l;   // default to gray
            g = l;
            b = l;
            v = (l <= 0.5) ? (l * (1.0 + sl)) : (l + sl - l * sl);

            if (v > 0)
            {
                double m;
                double sv;
                int sextant;
                double fract, vsf, mid1, mid2;

                m = l + l - v;
                sv = (v - m) / v;
                h *= 6.0;
                sextant = (int)h;
                fract = h - sextant;
                vsf = v * sv * fract;
                mid1 = m + vsf;
                mid2 = v - vsf;

                switch (sextant)
                {
                    case 0:
                        r = v;
                        g = mid1;
                        b = m;
                        break;
                    case 1:
                        r = mid2;
                        g = v;
                        b = m;
                        break;
                    case 2:
                        r = m;
                        g = v;
                        b = mid1;
                        break;
                    case 3:
                        r = m;
                        g = mid2;
                        b = v;
                        break;
                    case 4:
                        r = mid1;
                        g = m;
                        b = v;
                        break;
                    case 5:
                        r = v;
                        g = m;
                        b = mid2;
                        break;
                }
            }

            ColorRGB rgb;
            rgb.R = Convert.ToByte(r * 255.0f);
            rgb.G = Convert.ToByte(g * 255.0f);
            rgb.B = Convert.ToByte(b * 255.0f);
            return rgb;

        }
    }

    public struct ColorRGB
    {
        public byte R;
        public byte G;
        public byte B;

        public ColorRGB(Color value)
        {
            this.R = value.R;
            this.G = value.G;
            this.B = value.B;
        }

        public static implicit operator Color(ColorRGB rgb)
        {
            Color c = Color.FromArgb(rgb.R, rgb.G, rgb.B);
            return c;
        }

        public static explicit operator ColorRGB(Color c)
        {
            return new ColorRGB(c);
        }
    }
}
